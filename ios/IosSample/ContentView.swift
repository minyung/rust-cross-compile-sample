import SwiftUI
import myframework

struct ContentView: View {
    var body: some View {
        RustLib.shared.printTestLog()

        return Text(RustLib.shared.getHelloString(name: "minyung", num: 2023) ?? "Hello, world!")
            .padding()
    }
}

struct ContentView_Previews: PreviewProvider {
    static var previews: some View {
        ContentView()
    }
}
