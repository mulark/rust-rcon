use rcon::{Connection, Error};

fn main() -> Result<(), Error> {
    let address = "localhost:1234";
    let mut conn = Connection::builder()
        .enable_factorio_quirks(true)
        .connect(address, "test")?;

    demo(&mut conn, "/c print('hello')")?;
    demo(&mut conn, "/c print('world')")?;
    println!("commands finished");

    Ok(())
}

fn demo(conn: &mut Connection, cmd: &str) -> Result<(), Error> {
    println!("request: {}", cmd);
    let resp = conn.cmd(cmd)?;
    println!("response: {}", resp);
    Ok(())
}
