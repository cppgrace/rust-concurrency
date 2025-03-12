use anyhow::{Ok, Result};
use std::{
    thread,
    sync::mpsc
};
//创建4个生产者
const NUM_PRODUCERS: usize = 4;
#[derive(Debug)]
#[allow(dead_code)]
struct Msg{
    idx:usize,
    value:usize,
}
impl Msg {
    fn new(idx:usize,value:usize)->Self{
        Self{idx,value}
    }
}
fn main()-> Result<()>{
    let (tx,rx) =  mpsc::channel();
    //创建producer
    for i in 0..NUM_PRODUCERS{
        let tx = tx.clone();
        thread::spawn(move|| producer(i,tx));
    }
    //创建consumer
    let consumer = thread::spawn(move|| {
        for msg in rx{
            println!("{:?}",msg);
        }
    });

    //需要获取到消费者的返回值，用于join（协同线程）
    //让主线程等待，
    consumer
        .join()
        .map_err(|e| anyhow::anyhow!("Thread join error :{:?}",e))?;
   
    Ok(())
}
//生产者，会无限循环下去，
fn producer(idx:usize,tx:mpsc::Sender<Msg>)->Result<()> {
    loop{ 
        let value = idx * 1000 + (rand::random::<f64>() * 1000.0) as usize;
        tx.send(Msg::new(idx, value))?;
        let sleep_time = rand::random::<u8>() as u64 * 10;
        thread::sleep(std::time::Duration::from_millis(sleep_time));
        if rand::random::<f64>() > 0.9 {
            println!("producer {} exit",idx);
            break;
        }
    }
    Ok(())
    
}