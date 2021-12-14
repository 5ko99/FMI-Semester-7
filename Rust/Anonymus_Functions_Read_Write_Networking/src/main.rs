use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::net::{TcpStream, UdpSocket};

struct Map<I, F, B> where
    I: Iterator,
    F: FnMut(I::Item) -> B
{
    iter: I,
    f: F,
}

//Adaptor for iterators
impl<I, F, B> Iterator for Map<I, F, B> where
    I: Iterator,
    F: FnMut(I::Item) -> B
{
    type Item = B;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|x| (self.f)(x))
    }
}

fn main() -> io::Result<()> {
    let map = Map {
        iter: vec![1, 2, 3].into_iter(),
        f: |x| x * 3,
    };

    let v = map.collect::<Vec<_>>();
    println!("{:?}", v);

    //Currying
    fn curry(a: u32) -> Box<dyn Fn(u32) -> u32> {
        Box::new(move |b| 2*a + 3*b)
    }

    println!("{}", curry(1)(2));

    //Read and Write

    let mut f = File::open("foo.txt")?;
    let mut buffer = [0; 10];

    // Може да прочетем само 10 байта
    f.read(&mut buffer)?;

    // let mut buffer = Vec::new();
    // // Може да прочетем целия файл
    // f.read_to_end(&mut buffer)?;
    //
    // // Или директно да четем в String
    // let mut buffer = String::new();
    // f.read_to_string(&mut buffer)?;
    // println!("{}",buffer);

    //Read fro StdIN
    use std::io::{self, Read};
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    println!("{}",buffer);

    //Write
    use std::fs::File;
    use std::io::Write;

    let mut f = File::create("foo.txt")?;
    f.write_all(buffer.as_ref())?;

    //There are also BufReader and BufWriter
    let f = File::open("log.txt")?;
    let mut reader = BufReader::new(f);

    let mut line = String::new();
    let len = reader.read_line(&mut line)?;
    println!("First line is {} bytes long", len);


    let mut stream = BufWriter::new(TcpStream::connect("127.0.0.1:34254").unwrap());

    for i in 1..10 {
        stream.write(&[i]).unwrap();
    }
    stream.flush().unwrap();

    //Networking
    let mut socket = UdpSocket::bind("google.com").unwrap();
    let mut buf = [0; 10];
    let (amt, src) = socket.recv_from(&mut buf).unwrap();

    // Редекларира `buf` като слайс от получените данни и ги праща в обратен ред.
    let buf = &mut buf[..amt];
    buf.reverse();
    socket.send_to(buf, &src).unwrap();
    Ok(())
}
