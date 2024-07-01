// use super::lru::LRUCache;
// use super::InstanceCachePolicy;

// use std::{
//     cell::RefCell,
//     cmp::Eq,
//     collections::HashMap,
//     fmt::Debug,
//     hash::Hash,
//     rc::Rc,
//     time::{SystemTime, UNIX_EPOCH},
// };

// // Duo缓存策略
// pub struct DuoCache<Payload: Eq + Hash + Clone + Debug> {
//     leader_list: LRUCache<Payload>,
//     wingman_list: LRUCache<Payload>,
//     history_list: HashMap<Payload, u64>, // 存储访问时间
// }

// impl<Payload: Eq + Hash + Clone + Debug> DuoCache<Payload> {
//     fn new(capacity_leader: usize, capacity_wingman: usize) -> Self {
//         DuoCache {
//             leader_list: LRUCache::new(capacity_leader),
//             wingman_list: LRUCache::new(capacity_wingman),
//             history_list: HashMap::new(),
//         }
//     }

//     // 快速缓存逻辑
//     fn fast_cache(&mut self, key: &Payload) {
//         if self.history_list.contains_key(key) {
//             // 已经存在历史记录，认为是多次读取，直接放入leader_list
//             self.leader_list.put(key.clone(), Box::new(|_| true));
//         } else {
//             // 第一次读取，不缓存，直接放到远程存储
//             // 这里假设远程存储的逻辑已经实现
//             // remote_storage.put(key.clone());
//         }
//         // 更新 history_list
//         let now = SystemTime::now()
//             .duration_since(UNIX_EPOCH)
//             .expect("Time went backwards")
//             .as_secs();
//         self.history_list.insert(key.clone(), now);
//     }

//     // 间隔缓存逻辑
//     fn interval_cache(&mut self, key: &Payload) {
//         if let Some(last_access_time) = self.history_list.get(key) {
//             let now = SystemTime::now()
//                 .duration_since(UNIX_EPOCH)
//                 .expect("Time went backwards")
//                 .as_secs();
//             let interval = now - *last_access_time;
//             if interval < INTERVAL_THRESHOLD {
//                 // 短间隔，放入leader_list
//                 self.leader_list.put(key.clone(), Box::new(|_| true));
//             } else if !is_periodic_interval(interval) {
//                 // 长间隔且非周期性，放入wingman_list
//                 self.wingman_list.put(key.clone(), Box::new(|_| true));
//             }
//         }
//     }

//     // // 重用缓存逻辑
//     // fn re_cache(&mut self, key: &Payload) {
//     //     // 这里需要实现预取相关容器的逻辑
//     //     // prefetch_related_containers(key);
//     // }
// }
// unsafe impl<Payload: Eq + Hash + Clone + Debug> Send for DuoCache<Payload> {}

// impl<Payload: Eq + Hash + Clone + Debug> InstanceCachePolicy<Payload> for DuoCache<Payload> {
//     fn get(&mut self, key: Payload) -> Option<Payload> {
//         if let Some(_) = self.leader_list.get(key.clone()) {
//             self.fast_cache(&key);
//             Some(key)
//         } else if let Some(_) = self.wingman_list.get(key.clone()) {
//             self.interval_cache(&key);
//             let _ = self.wingman_list.remove_all(&key);
//             self.leader_list.put(key.clone(), Box::new(|_| true));
//             Some(key)
//         } else {
//             None
//         }
//     }

//     fn put(
//         &mut self,
//         key: Payload,
//         can_be_evict: Box<dyn FnMut(&Payload) -> bool>,
//     ) -> (Option<Payload>, bool) {
//         let result = self.leader_list.put(key.clone(), can_be_evict.clone());
//         if !result.1 {
//             let result = self.wingman_list.put(key, can_be_evict);
//             if !result.1 {
//                 return result;
//             }
//         }
//         result
//     }

//     fn remove_all(&mut self, key: &Payload) -> bool {
//         if self.leader_list.remove_all(key) {
//             return true;
//         } else if self.wingman_list.remove_all(key) {
//             return true;
//         }
//         false
//     }
// }

// // 假设的辅助函数，需要根据实际情况实现
// const INTERVAL_THRESHOLD: u64 = 60; // 间隔阈值，单位：秒
// fn is_periodic_interval(interval: u64) -> bool {
//     // 判断间隔是否周期性，需要自定义算法
//     false
// }
