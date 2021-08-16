pub struct BackOff {
    count: u64,
}

#[allow(clippy::new_without_default)]
impl BackOff {


    pub fn new() -> BackOff {
        BackOff {
            count: 0,
        }
    }


    pub fn reset(&mut self) -> u64 {
        self.count = 0;
        self.count
    }


    pub async fn wait(&mut self) -> u64 {
        async_std::task::sleep(
            std::time::Duration::from_millis(self.count * self.count)
        ).await;
        if self.count <= 50 {
            self.count += 1;
        }
        self.count - 1
    }
}



