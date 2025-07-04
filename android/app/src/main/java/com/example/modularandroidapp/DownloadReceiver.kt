package com.example.modularandroidapp

import android.content.BroadcastReceiver
import android.content.Context
import android.content.Intent
import android.app.DownloadManager
import android.util.Log

class DownloadReceiver : BroadcastReceiver() {
    companion object {
        private const val TAG = "DownloadReceiver"
    }

    override fun onReceive(context: Context?, intent: Intent?) {
        if (intent?.action == DownloadManager.ACTION_DOWNLOAD_COMPLETE) {
            val downloadId = intent.getLongExtra(DownloadManager.EXTRA_DOWNLOAD_ID, -1)
            
            if (downloadId != -1L) {
                Log.d(TAG, "Download completed: $downloadId")
                
                // You can add additional processing here
                // For example, verify the downloaded file, move it to the correct location, etc.
                
                // Notify the user or update the UI
                // This could be done through a callback or event bus
            }
        }
    }
} 