#![allow(warnings, unused)]

trait Address {
}

impl Address for u32 {

}

trait Process { }

struct IterReader { }  // read forwards, backwards, etc until we can't.

trait Read {
    fn read(&self, process: &Process, addr: Address) -> IterReader;
}

trait Write {
    fn write(&mut self, process: &Process, addr: Address, bytes: &[u8]);
}

trait Driver {

}

// 
struct WinRpmDriver { }

impl Driver for WinRpmDriver {

}

// 
struct WinCapcomDriver { }

impl Driver for WinCapcomDriver {

}

// TODO: in the docs/book, create a flowchart for selecting drivers.


//let inspector = ...
//    .find(| process_name, executable_name, window_name, ... | {
//        process_name == "target.exe" || window_name == "Target Name"
//    })
//    .attach();

fn main() {
    // drivers are the thing that handles interacting with a process. for simplicity, we're only
    // supporting read/write
    let driver = WinRpmDriver::new();

    // quick iter to find a process, might be better to use some other system info lib
    let address_space = AddressSpace::iter_processes(&driver)  // TODO: maybe make this more generic, so we can scan stuff like files or tutorial buffers in the book?
        find(| process_metadata | true /* find one by returning true */ )
        .attach();

    // Locate a simple byte pattern with an offset
    some_pattern = Scannner::new()
        .pattern("90 90 ? ? ? 8b 8c 8? 88") // basic ida/ghidra pattern, will select anything at the start of the pattern
        .offset(0xff);  // now we'll select anything 0xff bytes after the pattern

    // Locate the pattern in our process's address space.
    for (idx, addr) in some_pattern.locate(&address_space).iter().enumerate() {
        println!("some pattern [{}]: {:?}", idx, addr);
    }

    // Or just locate a single arbitrary result
    let addr = some_pattern.locate(&address_space).pop();
    println!("some pattern (single): {:?}", addr);

    // Interesting example that demonstrates the tree-oriented scanning structure
    nt_header_locator = Scannner::in_space(&address_space)
        // Locate the nt 
        .by_pattern("90 90 ? ? ? 8b 8c 8? 88");

    pe_header_locator = nt_header_locator.clone()
        .by_pattern("90 90 ? ? ? 8b 8c 8? 88");

    imports = nt_header_locator.clone()
        .by_pattern("90 90 ? ? ? 8b 8c 8? 88");

    world_objects = nt_header_locator.clone()
        .by_pattern("90 90 ? ? ? 8b 8c 8? 88");

    map_objects = nt_header_locator.clone()
        .by_pattern("90 90 ? ? ? 8b 8c 8? 88");

    // On demand read of raw bytes
    let world = address_space.read(world_objects.first());

    // On demand read of typed object (must impl Object)
    #[derive(mio::Object)]
    struct MapObject {

    }

    let map = MapObject::at_address(map_objects.first());
    println!("{:?}", map_object.read(&address_space));

    // calculate most efficient scan pattern, then go execute and return the typed objects
    let (world, map) = address_space.batch(world, map).read();

    // TODO: quick_object macro
    // TODO: support "location" string insertion in pattern strings, so we can compose by doing e.g. "90 90 ? ? {nested_scanner_starting_here} ? ?". String generator inserts valid syntax for a nexted scan that saves intermediate symbols
}
