use std::sync::{Arc, Mutex};
use std::collections::HashMap;

pub struct NotificationManager {
    notifications: Arc<Mutex<HashMap<i32, NotificationData>>>,
    next_id: Arc<Mutex<i32>>,
    callback: Option<Box<dyn Fn(i32, String, String, String) + Send + Sync>>,
}

#[derive(Clone)]
pub struct NotificationData {
    pub id: i32,
    pub title: String,
    pub message: String,
    pub timestamp: String,
    pub priority: NotificationPriority,
}

#[derive(Clone)]
pub enum NotificationPriority {
    Low,
    Normal,
    High,
    Urgent,
}

impl NotificationManager {
    pub fn new() -> Self {
        Self {
            notifications: Arc::new(Mutex::new(HashMap::new())),
            next_id: Arc::new(Mutex::new(1)),
            callback: None,
        }
    }
    
    pub fn set_callback(&mut self, callback: impl Fn(i32, String, String, String) + Send + Sync + 'static) {
        self.callback = Some(Box::new(callback));
    }
    
    pub fn send_notification(&self, title: String, message: String, priority: NotificationPriority) -> i32 {
        let id = {
            let mut next_id = self.next_id.lock().unwrap();
            let current_id = *next_id;
            *next_id += 1;
            current_id
        };
        
        let timestamp = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
        
        let notification = NotificationData {
            id,
            title: title.clone(),
            message: message.clone(),
            timestamp: timestamp.clone(),
            priority,
        };
        
        // Store notification
        {
            let mut notifications = self.notifications.lock().unwrap();
            notifications.insert(id, notification);
        }
        
        // Call callback to notify Android
        if let Some(callback) = &self.callback {
            callback(id, title, message, timestamp);
        }
        
        id
    }
    
    pub fn send_sensor_alert(&self, sensor_name: &str, value: f32, threshold: f32) {
        let title = format!("Sensor Alert: {}", sensor_name);
        let message = format!("Value {} exceeded threshold {}", value, threshold);
        self.send_notification(title, message, NotificationPriority::High);
    }
    
    pub fn send_system_notification(&self, title: String, message: String) {
        self.send_notification(title, message, NotificationPriority::Normal);
    }
    
    pub fn get_notification(&self, id: i32) -> Option<NotificationData> {
        let notifications = self.notifications.lock().unwrap();
        notifications.get(&id).cloned()
    }
    
    pub fn get_all_notifications(&self) -> Vec<NotificationData> {
        let notifications = self.notifications.lock().unwrap();
        notifications.values().cloned().collect()
    }
    
    pub fn clear_notification(&self, id: i32) -> bool {
        let mut notifications = self.notifications.lock().unwrap();
        notifications.remove(&id).is_some()
    }
    
    pub fn clear_all_notifications(&self) {
        let mut notifications = self.notifications.lock().unwrap();
        notifications.clear();
    }
} 