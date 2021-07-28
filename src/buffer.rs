pub type Page = [u8; PAGE_SIZE];
pub struct Buffer {
    pub page_id: PageId,
    pub page: RefCell<Page>,
    pub is_dirty: Cell<bool>,
}
pub struct Frame {
    usage_count: u64,
    buffer: Rc<Buffer>,
}
pub struct BufferPool {
    buffers: Vec<Frame>,
    next_victim_id: BufferId,
}
pub struct BufferPoolManager {
    disk: DiskManager,
    pool: BufferPool,
    page_table: HashMap<PageId, BufferId>,
}

fn evict(&mut self) -> Option<BufferId> {
    let pool_size = self.size();
    let mut consecutive_pinned = 0;
    // バッファプール内のすべてのバッファを巡回すするループ
    let pool_size = loop {
        let next_victim_id = self.next_victim_id;
        let frame = &mut self[next_victim_id];
        // バッファを捨てるか否かの条件: バッファを利用するとusage_count +1
        if frame.usage_count == 0 {
            break slef.next_victim_id;
        }
        // 巡回中のバッファが貸出中でないかどうかの条件: 貸出中でないならusage_count -1
        if Rc::get_mut(&mut frame.buffer).id_some() {
            frame.usage_count -= 1;
            consecutive_pinned = 0;
        } else {
            // バッファが貸出中のとき: consecutive_pinned +1
            consecutive_pinned += 1;
            // consecutive_pinnedがバッファプールのサイズと同じ(=すべてのバッファが貸出中)のとき
            if consecutive_pinned >= pool_size {
                return None;
            }
        }
        self.next_victim_id = self.increment_id(self.next_victim_id);
    };
    Some(victim_id)
}
fn increment_id(&self, buffer_id: BufferId) -> BufferId {
    Bufferid((buffer_id.0 + 1) % self.size())
}
fn fetch_page(&mut self, page_id: PageId) -> Result<Result<Buffer>, Error> {
    // ページがバッファプールにある場合
    if let Some(&buffer_id) = self.page_table.get(&page_id) {
        let frame = &mut self.pool[buffer_id];
        frame.usage_count += 1;
        return Ok(frame.bufer.clone());
    }
    // ページがバッファプールにない場合
    // 読み込むページお格納するバッファの決定
    let buffer_id = self.pool.evict().ok_or(Error::NoFreeBuffer)?;
    let frame = &mut self.pool[buffer_id];
    let evict_page_id = frame.buffer.page_id;
    {
        let buffer = Rc::get_mut(&mut frame.buffer).unwrap();
        // is_dirty: バッファの内容が変更されており,ディスクの内容が古くなっていることを示すフラグ
        // is_dirtyがtrueの場合そのバッファをディスクに書き出す
        if buffer.is_dirty.get() {
            self.disk
                .write_page_data(evict_page_id, buffer.page.get_mut());
        }
        buffer.page_id = page_id;
        buffer.is_dirty.set(false);
        // ページを読み出す
        self.disk.read_page_data(page_id, buffer.page.get_mut())?;
        frame.usage_count = 1;
    }
    let page = Rc::clone(&frame.buffer);
    // ページ読み出しの際にバッファにはいっているページが入れ替わったためページテーブルを更新する
    self.page_table.remove(&evict_page_id);
    self.page_table.insert(page_id, buffer_id);
}
