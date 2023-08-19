import .models.character
import .models.stats
import .models.identity

character.Character {
    name = "Shrimp"
    stats = stats.Mag("INT") {
        class = "Magician"
        base = stats.Base {
            FOR=3
            AGI=3
            REA=3
            CON=5
            CHA=3
            LOG=3
            INT=5
            VOL=7
            Edge=2
            AddInit=3
        }
        MAG=12
        Init= 7
    }
    identities = [
        identity.Identity {
            alias = "Laurence Guinvite"
            quality: "i4"
            price: 5000
            description: "dsf dfgdfg dfgdfg"
            lifestyle: "Squatteur"
            nuyens: None
        }
    ]
}


