//TCP echo server 
//
//very rudimentary implementation of a TCP broadcast message to all clients when recieved from one
//client 
use tokio::{net::TcpListener,io::{AsyncBufReadExt,BufReader,AsyncWriteExt},sync::broadcast};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("localhost:8080").await.unwrap();//wait until socket is bound and ready for response

    let(tx,_rx) =broadcast::channel(10);//this enables mpmc

    loop {//this loop enable mutiple clients
        let (mut socket ,addr) = listener.accept().await.unwrap();

        let tx = tx.clone();//cloning is necessary so the thread can be moved
        let mut rx = tx.subscribe();//reciever subscribed to tx

        tokio::spawn(async move{
            let (reader,mut writer) = socket.split();//splits socket into read and write half , writer is kept mutable for obvious reasons
            let mut reader = BufReader::new(reader);
            let mut line = String::new();

            loop{ //this loop is for every input 
                tokio::select! {//select is a tokio macro that allows us to run mutiple async tasks and act on the first on that finishes
                    result=reader.read_line(&mut line)=>{ //read_line() just appends to the buffer hence line.clear() is used on 31 to clear buffer content beforehand
                        if result.unwrap()==0{
                            break;
                        }
                        tx.send((line.clone(),addr)).unwrap();//sending the line to the server along with the address
                        line.clear();
                    }
                    result=rx.recv() => {
                        let(msg,other_addr) =result.unwrap();//recie message and address

                        if addr != other_addr{//this makes sure client doesn't get their own message back , and is accomplished by comparing messages
                            writer.write_all(msg.as_bytes()).await.unwrap()//write only the reciv client
                        }
                    }
                }
            }
        });

    }



}
