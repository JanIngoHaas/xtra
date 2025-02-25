use xtra::prelude::*;

#[derive(Default, xtra::Actor)]
struct Printer {
    times: usize,
}

struct Print(String);

#[async_trait]
impl Handler<Print> for Printer {
    type Return = ();

    async fn handle(&mut self, print: Print, _ctx: &mut Context<Self>) {
        self.times += 1;
        println!("Printing {}. Printed {} times so far.", print.0, self.times);
    }
}

#[tokio::main]
async fn main() {
    let addr = xtra::spawn_tokio(Printer::default(), Mailbox::unbounded());

    let addr2 = addr.clone();
    
    loop {
        addr.send(Print("hello".to_string()))
            .await
            .expect("Printer should not be dropped");

        addr2.send(Print("hello2".to_string())).await.unwrap();        
    }
}
