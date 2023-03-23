import XCTest
@testable import myframework

class MyframeworkTests: XCTestCase {

    override func setUpWithError() throws {
        // Put setup code here. This method is called before the invocation of each test method in the class.
    }

    override func tearDownWithError() throws {
        // Put teardown code here. This method is called after the invocation of each test method in the class.
    }

    func testRustLibGetHelloString() throws {
        let result = RustLib.shared.getHelloString(name: "minyung", num: 2023)
        XCTAssertEqual(result, "Hello, minyung 2023")
    }

    func testPerformanceExample() throws {
        // This is an example of a performance test case.
        self.measure {
            // Put the code you want to measure the time of here.
        }
    }

}
