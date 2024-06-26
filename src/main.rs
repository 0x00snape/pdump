mod modules;

fn main() {

    // Restart self with sudo if needed
    sudo::escalate_if_needed().unwrap();

    let args: Vec<_> = std::env::args().collect();

    if args.len() != 2 {
        println!("Usage: sudo ./scanme <PROCESS_NAME>");
        std::process::exit(0);
    }

    // Getting the PID
    let pid = modules::getPID(args[1].clone());
    
    let (start, end) = modules::heapADDR(pid);
    modules::readMEM(pid, start, end);

}
