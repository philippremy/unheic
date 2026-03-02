#include <libUnHEIC.hxx>

#include <exception>

/* Forwards an unhandled C++ exception to Rust and initate a panic */
void forward_exception_to_rust() noexcept {
    std::exception_ptr exception_ptr = std::current_exception();

    std::exception outer;

    const char* what;
    if(!exception_ptr)
        what = "";

    try {
        std::rethrow_exception(exception_ptr);
    } catch(std::exception inner) {
        outer = inner;
    }

    what = outer.what();

    transfer_exception_to_rust(what);
}

/* The actual binary entry point */
int main(int argc, char** argv) {

    // Set the global exception handler
    std::set_terminate(forward_exception_to_rust);

    // Call back into Rust an start the app
    return start_unheic(argc, argv);
}
