use std::{fs::File, io::BufReader, net::UdpSocket};

mod dns;

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:53")?;
    let mut buf = [0; 512];

    loop {
        println!("Listening...");

        let (amt, src) = socket.recv_from(&mut buf)?;
        let vec_buf = buf[..amt].to_vec();

        println!("Incoming request from {src}");

        let mut header = dns::Header::from_buffer(&vec_buf, 0);

        let mut idx = 12;
        let mut queries = Vec::<dns::Query>::new();
        let mut answers = Vec::<dns::ResourceRecord>::new();
        for _i in 0..header.qdcount {
            let query = dns::Query::from_buffer(&vec_buf, idx);
            idx += 2 + 2 + query.qname.len();

            let filename = format!("zones/{}zone", &query.get_name());
            let reader = BufReader::new(File::open(filename)?);

            let json: serde_json::Value = serde_json::from_reader(reader)?;
            let arecords = json["a"].as_array().unwrap();
            let ttl = json["$ttl"].as_u64().unwrap() as u32;

            arecords.iter().for_each(|record| {
                answers.push(dns::ResourceRecord::new(
                    ttl,
                    record["value"].as_str().unwrap(),
                ));
            });

            queries.push(query);
        }

        // Response building
        let mut response = Vec::<u8>::new();

        header.ancount = answers.len() as u16;
        header.nscount = 0;
        header.arcount = 0;
        response.append(&mut header.to_buffer());

        queries.iter().for_each(|query| {
            response.append(&mut query.to_buffer());
        });

        answers.iter().for_each(|answer| {
            response.append(&mut answer.to_buffer());
        });

        socket.send_to(response.as_slice(), &src)?;
        println!("Sent response to {src}");
    }
}
