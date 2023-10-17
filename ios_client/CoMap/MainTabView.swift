import SwiftUI

struct MainTabView: View {
    @State private var selection = 0
    
    
    var body: some View {
        
        ZStack {
            // Main View
                
            TabView(selection: $selection) {
                
                GameMapView()
                    .tabItem{
                        Image(systemName: "map.fill")
                        Text("Pinnect")
                    }
                    .tag(1)
                
                NavigationStack {
                    ScrollView {
                        ForEach(0..<20) { _ in
                            ContentCardView()
                        }
                    }
                    .navigationTitle("Collection")
                }
                .tabItem{
                    Image(systemName: "star.fill")
                    Text("Collection")
                }
                .tag(2)
                
                SettingsView()
                    .tabItem{
                        Image(systemName: "gear")
                        Text("Settings")
                    }
                    .tag(3)

            }.background(Color.white)
        }
    }
}
