import SwiftUI

struct TopicCardView: View {
    
    var imageName: String
    var name: String

    var body: some View {
        VStack(alignment: .leading) {
            GeometryReader { geometry in
                Image(imageName)
                    .resizable()
                    .aspectRatio(contentMode: .fill)
                    .frame(height: 100)
                    .clipShape(RoundedRectangle(cornerRadius: 10, style: .continuous))
                    .padding(.vertical, 12)
            }
            Text(name).font(.title3).bold()
                .padding(.horizontal, 4)
                .padding(.bottom, 4)
        }
    }
}

struct SettingsView: View {
    @State private var selection = 0
    let cards = [
        ["title": "the_legend_of_zelda_breath_of_the_wild", "name": "The Legend of Zelda: Breath of the Wild"],
        ["title": "red_dead_redemption_2", "name": "Red Dead Redemption 2"],
        ["title": "the_elder_scrolls_V", "name": "The Elder Scrolls V: Skyrim"],
        ["title": "super_mario_odyssey", "name": "Super Mario Odyssey"],
        ["title": "pokemon", "name": "Pokémon Sword Shield"],
        ["title": "animal_crossing", "name": "Animal Crossing"],
        ["title": "super_smash_bros", "name": "Super Smash Bros"]
        // Add your other cards here
    ]
    
    var body: some View {
        NavigationView {
            Form {
                Section {
                    VStack(alignment: .leading) {
                        GeometryReader { geometry in
                            Image("bubble")
                                .resizable()
                                .aspectRatio(contentMode: .fill)
                                .frame(width: geometry.size.width)
                                .clipShape(RoundedRectangle(cornerRadius: 10, style: .continuous))
                                .padding(.vertical, 12)
                            
                        }
                        Text("Bubble Observers").font(.title3).bold()
                            .padding(.horizontal, 4)
                            .padding(.bottom, 4)
                        Text("""
The entire book unfolds through the exploration and journey of two ghost bubbles.
The ghost bubbles come from the god "One" of a symbiotic multi-self entity from the future, who predicts that the future civilization will inevitably head towards a great collapse. Therefore, it dispatches computing sprites to simulate different possibilities in parallel universes, while also sending ghost bubbles to trace history as observers, seeking the reasons from the past.
""")
                        .padding(.horizontal, 4)
                        .padding(.bottom, 4)

                        Spacer()

                    }
                }
                
                Section {
                    ForEach(cards, id: \.self) { card in
                        TopicCardView(imageName: "title_\(card["title"]!)", name: card["name"]!)
                    }
                }
                
                Section {
                    HStack {
                        Image(systemName: "plus.circle.fill")
                        Text("Create new topic")
                    }
                    .font(.title3.bold())
                }
                
                Section {
                    Button(action: {
                        // action for Join Curator button
                    }) {
                        Text("Connect Wallet")
                            .bold()
                            .frame(minWidth: 0, maxWidth: .infinity)
                            .frame(height: 24)
                            .padding()
                            .foregroundColor(Color("controlWithBackground"))
                            .background(Color("control"))
                            .clipShape(RoundedRectangle(cornerRadius: 16, style: .continuous))
                    }

                }
            }
            .navigationTitle("Switch Topics")
        }
    }
}
