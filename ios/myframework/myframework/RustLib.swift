import Foundation

public class RustLib {
    public static let shared = RustLib()

    private init() {
        rust_init()
    }

    public func printTestLog() {
        print_test_log()
    }

    public func getHelloString(name: String, num: Int32) -> String? {
        let result = get_hello_string((name as NSString).utf8String, num)
        if let result = result {
            let str = String(cString: result)
            free_hello_string(result)
            return str
        }
        return nil
    }
}
