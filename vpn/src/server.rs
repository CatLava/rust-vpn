
pub fn server_establishment(){
    let listener = TcpListener::bind("0.0.0.0:12345").unwrap();

    // keep track of connected clients
    // Use Arc and Mutex to ensure it can be modified in different threads
    let clients: Arc<Mutex<HashMap<usize, TcpStream>>> = Arc::new(Mutex::new(HashMap::new()));

    // create a TUN interface to control network connections
    // this is not cross-platform probably want to do this on linux
    // https://docs.rs/crate/tun/latest
    let mut config = tun::Configuration::defaul();
    config.name("tun0");
    let tun_device = tun::create(&config).unwrap();

    if let Err(e) = setup_tun_interface() {
        eprintln!("Failed to set up TUN interface: {}", e);
        return;
    }

    info!("Server started on 0.0.0.0:12345");

    // Spawn a thread to fetch client keys and read data from interface

    let tun_device_clone = shared_tun.clone();
    let clients_clone = clients.clone();

    thread::spawn(move || {
        let clients_guard = clients_clone.lock().unwrap();
        // clone and read client TCPstream
        if let Some(client) = clients_guard.get(&0) {
            if let Ok(client_clone) = client.try_clone() {
                drop(clients_guard);
                let mut locked_tun = tun_device_clone.lock().unwrap();
                read_from_tun_and_send_to_client(&mut *locked_tun, client_clone);

            } else {
                println!("Failed ot clone client TcpSream");
            }
        }
    });


}