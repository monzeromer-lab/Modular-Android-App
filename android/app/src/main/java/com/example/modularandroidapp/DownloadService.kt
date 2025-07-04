package com.example.modularandroidapp

import android.app.DownloadManager
import android.content.Context
import android.net.Uri
import android.os.Environment
import android.util.Log
import java.io.File
import java.io.FileInputStream
import java.io.FileOutputStream
import java.io.IOException

class DownloadService(private val context: Context) {
    companion object {
        private const val TAG = "DownloadService"
    }

    suspend fun downloadLibrary(url: String): File? {
        return try {
            Log.d(TAG, "Starting download from: $url")
            
            val downloadManager = context.getSystemService(Context.DOWNLOAD_SERVICE) as DownloadManager
            
            val request = DownloadManager.Request(Uri.parse(url))
                .setTitle("Library Update")
                .setDescription("Downloading updated library")
                .setNotificationVisibility(DownloadManager.Request.VISIBILITY_VISIBLE_NOTIFY_COMPLETED)
                .setDestinationInExternalFilesDir(context, Environment.DIRECTORY_DOWNLOADS, "libmainlogic_temp.so")
                .setAllowedOverMetered(true)
                .setAllowedOverRoaming(true)

            val downloadId = downloadManager.enqueue(request)
            
            // Wait for download to complete
            val success = waitForDownload(downloadManager, downloadId)
            
            if (success) {
                val downloadedFile = File(context.getExternalFilesDir(Environment.DIRECTORY_DOWNLOADS), "libmainlogic_temp.so")
                if (downloadedFile.exists()) {
                    Log.d(TAG, "Download completed successfully")
                    downloadedFile
                } else {
                    Log.e(TAG, "Downloaded file not found")
                    null
                }
            } else {
                Log.e(TAG, "Download failed")
                null
            }
        } catch (e: Exception) {
            Log.e(TAG, "Error downloading library", e)
            null
        }
    }

    private suspend fun waitForDownload(downloadManager: DownloadManager, downloadId: Long): Boolean {
        return try {
            var completed = false
            var attempts = 0
            val maxAttempts = 60 // 5 minutes timeout
            
            while (!completed && attempts < maxAttempts) {
                val query = DownloadManager.Query().setFilterById(downloadId)
                val cursor = downloadManager.query(query)
                
                if (cursor.moveToFirst()) {
                    val status = cursor.getInt(cursor.getColumnIndex(DownloadManager.COLUMN_STATUS))
                    
                    when (status) {
                        DownloadManager.STATUS_SUCCESSFUL -> {
                            completed = true
                        }
                        DownloadManager.STATUS_FAILED -> {
                            val reason = cursor.getInt(cursor.getColumnIndex(DownloadManager.COLUMN_REASON))
                            Log.e(TAG, "Download failed with reason: $reason")
                            return false
                        }
                        DownloadManager.STATUS_PAUSED -> {
                            Log.d(TAG, "Download paused")
                        }
                        DownloadManager.STATUS_PENDING -> {
                            Log.d(TAG, "Download pending")
                        }
                        DownloadManager.STATUS_RUNNING -> {
                            val bytesDownloaded = cursor.getLong(cursor.getColumnIndex(DownloadManager.COLUMN_BYTES_DOWNLOADED_SO_FAR))
                            val bytesTotal = cursor.getLong(cursor.getColumnIndex(DownloadManager.COLUMN_TOTAL_SIZE_BYTES))
                            if (bytesTotal > 0) {
                                val progress = (bytesDownloaded * 100 / bytesTotal).toInt()
                                Log.d(TAG, "Download progress: $progress%")
                            }
                        }
                    }
                }
                
                cursor.close()
                
                if (!completed) {
                    kotlinx.coroutines.delay(5000) // Wait 5 seconds before checking again
                    attempts++
                }
            }
            
            completed
        } catch (e: Exception) {
            Log.e(TAG, "Error waiting for download", e)
            false
        }
    }

    fun copyFile(source: File, destination: File): Boolean {
        return try {
            FileInputStream(source).use { input ->
                FileOutputStream(destination).use { output ->
                    input.copyTo(output)
                }
            }
            true
        } catch (e: IOException) {
            Log.e(TAG, "Error copying file", e)
            false
        }
    }
} 