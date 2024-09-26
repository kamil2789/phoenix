
pub type ID = u32;

#[derive(Default)]
pub struct IdGarbageCollector {
    num_pool: u32,
    renewable_ids: Vec<ID>,
}

impl IdGarbageCollector {
    pub fn create_id(&mut self) -> ID {
        if let Some(id) = self.renewable_ids.pop() {
            id
        } else {
            self.num_pool += 1;
            self.num_pool
        }
    }

    #[must_use]
    #[allow(dead_code)]
    pub fn get_num_pool(&self) -> u32 {
        self.num_pool
    }

    #[must_use]
    #[allow(dead_code)]
    pub fn get_renewable_ids_num(&self) -> u32 {
        self.renewable_ids.len() as u32
    }

    pub fn remove_id(&mut self, id: ID) {
        self.renewable_ids.push(id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_id_garbage_collector() {
        let mut id_gc = IdGarbageCollector::default();
        let id1 = id_gc.create_id();
        let id2 = id_gc.create_id();
        let id3 = id_gc.create_id();

        assert_eq!(id1, 1);
        assert_eq!(id2, 2);
        assert_eq!(id3, 3);
        assert_eq!(id_gc.get_num_pool(), 3);
        assert_eq!(id_gc.get_renewable_ids_num(), 0);

        id_gc.remove_id(id2);
        assert_eq!(id_gc.get_num_pool(), 3);
        assert_eq!(id_gc.get_renewable_ids_num(), 1);

        let id4 = id_gc.create_id();
        assert_eq!(id4, id2);
        assert_eq!(id_gc.get_num_pool(), 3);
        assert_eq!(id_gc.get_renewable_ids_num(), 0);
    }
}
