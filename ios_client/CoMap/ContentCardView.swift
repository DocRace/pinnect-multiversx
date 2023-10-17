import SwiftUI

struct ContentCardView: View {
    var body: some View {
        VStack(alignment: .leading, spacing: 10) {
            Text(sampleCards.randomElement()!.title)
                .font(.title).bold()
            HStack {
                Image("avatar\(Int.random(in: 1...13))")
                    .resizable()
                    .frame(width: 54, height: 54)
                    .clipShape(Circle())
                    .padding(.bottom, 8)
                VStack(alignment: .leading) {
                    Text("Race Li")
                        .font(.headline)
                    Text("08/08/2023")
                        .font(.subheadline)
                        .foregroundColor(.secondary)
                }
            }
            
            Image("random\(Int.random(in: 1...30))")
                .resizable()
                .scaledToFill()
                .frame(height: 200)
                .clipped()
                .clipShape(RoundedRectangle(cornerRadius: 10, style: .continuous))
                .padding(.bottom, 8)
            
            Text("""
Rick and Morty were in the garage working on Rick's latest invention. "Pass me the quantum carburetor Morty," said Rick. Morty handed him the strange device.
Suddenly a portal opened up and another Rick and Morty stepped out. "Hey, we're you from another dimension," said the new Rick.
"Wow, I can't believe there are alternate versions of us," said Morty.
"Yeah there's infinite timelines kid, some nearly identical others completely different," explained Rick.
Just then a Cromulon alien ship appeared overhead. "Show me what you got!" it bellowed.
The two Ricks looked at each other and nodded. They pulled out laser guns and blasted the ship out of the sky before it could attack.
"Wubba lubba dub dub!" cheered the Ricks. The Mortys just stood there amazed at what they had witnessed.
""")
                .font(.body)
                .padding(.bottom, 8)
            
            Image("random\(Int.random(in: 1...30))")
                .resizable()
                .scaledToFill()
                .frame(height: 200)
                .clipped()
                .clipShape(RoundedRectangle(cornerRadius: 10, style: .continuous))
                .padding(.bottom, 8)
            
            HStack {
                Image("map")
                    .resizable()
                    .scaledToFill()
                    .frame(width: 80, height: 40)
                Spacer()
                
                HStack {
                    Image(systemName: "heart")
                    Text("\(Int.random(in: 30...500))")
                }
                HStack {
                    Image(systemName: "message")
                    Text("\(Int.random(in: 3...128))")
                }
            }
        }
        .padding()
        .background(Color(.systemGray6))
        .cornerRadius(10)
        .padding()
    }
}

let sampleCards: [Card] = [
    Card(
        user: "Rick",
        creationTime: Date(),
        text: "In an alternate dimension, Morty and I discovered a world entirely populated by sentient pizzas. The pizzas were living in fear of being eaten by a race of giant humans. It was a cheesy nightmare, literally.",
        title: "Pizza Nightmare Dimension",
        isTrend: true,
        isRandom: false,
        isSelected: false,
        commentCount: 15,
        likes: 100
    ),
    Card(
        user: "Morty",
        creationTime: Date(),
        text: "Rick and I once traveled to a dimension where everything was drawn by a four-year-old. The laws of physics were all over the place. I still have nightmares about the scribble monsters.",
        title: "Child-drawn Dimension",
        isTrend: false,
        isRandom: true,
        isSelected: false,
        commentCount: 20,
        likes: 80
    ),
    Card(
        user: "Summer",
        creationTime: Date(),
        text: "One day, Rick gave me a device that could change the color of anything. I turned the whole world pink before I realized it wasn't a dream. Good times.",
        title: "World Changing Color Device",
        isTrend: false,
        isRandom: false,
        isSelected: true,
        commentCount: 12,
        likes: 120
    ),
    // More card instances
    Card(
        user: "Beth",
        creationTime: Date(),
        text: "The time we had Morphizer-XE... I turned into a giant monster mom. It was not my finest hour.",
        title: "Giant Monster Mom Incident",
        isTrend: true,
        isRandom: false,
        isSelected: false,
        commentCount: 18,
        likes: 150
    ),
    Card(
        user: "Jerry",
        creationTime: Date(),
        text: "Remember when I got that alien parasite stuck to my face? That sucked.",
        title: "Alien Parasite Incident",
        isTrend: false,
        isRandom: true,
        isSelected: false,
        commentCount: 22,
        likes: 70
    ),
    Card(
        user: "Rick",
        creationTime: Date(),
        text: "Ever heard of a dimension where everyone communicates through burps? Yeah, didn't think so.",
        title: "Burping Communication Dimension",
        isTrend: false,
        isRandom: false,
        isSelected: true,
        commentCount: 20,
        likes: 170
    ),
    Card(
        user: "Morty",
        creationTime: Date(),
        text: "There's this one time when Rick gave me a love potion. It escalated quickly.",
        title: "Love Potion Incident",
        isTrend: true,
        isRandom: false,
        isSelected: false,
        commentCount: 28,
        likes: 90
    ),
    Card(
        user: "Summer",
        creationTime: Date(),
        text: "I once got locked in a Mad Max style post-apocalyptic world. That was crazy.",
        title: "Post-apocalyptic World Adventure",
        isTrend: false,
        isRandom: true,
        isSelected: false,
        commentCount: 32,
        likes: 140
    ),
    // Continue creating more Card instances until there are 20 of them...
]

// Card model
struct Card: Identifiable {
    let id = UUID()
    let user: String
    let creationTime: Date
    let text: String
    let title: String
    let isTrend: Bool
    let isRandom: Bool
    let isSelected: Bool
    let commentCount: Int
    let likes: Int
}



#Preview {
    return ContentCardView()
}

