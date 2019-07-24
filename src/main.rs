extern crate realm;
extern crate graftpress_lib;

realm::realm!{
    graftpress_lib::forward::magic,
    if sys.argv == "build" {
        do_build();
        return;
    }
}
