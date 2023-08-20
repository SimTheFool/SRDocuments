schema Effect:
    type: str
    name: str
    tags: [str]
    description: str
    transform: any

schema Stats:
    CON: int
    VOL: int
    INT: int

schema Character:
    name: str
    base_stats: Stats
    effects: [Effect]

    #_map_effect: any = for i in effects: i.transform

    final_stats: Stats

wicca = Effect {
    type = "metamagic",
    name = "Wicca tradition",
    tags = ["holy", "divine"],
    description = "Blessed by the gods"
    transform = lambda input: Stats -> Stats {
        Stats {
            CON = input.CON + 1,
            VOL = input.VOL + 1,
            INT = input.INT + 3
        }
    }
}

wicca2 = Effect {
    type = "metamagic",
    name = "Wicca tradition",
    tags = ["holy", "divine"],
    description = "Blessed by the gods"
    transform = lambda input: Stats -> Stats {
        Stats {
            CON = input.CON,
            VOL = input.VOL,
            INT = input.INT + 10
        }
    }
}


Character {
    name = "Alusia",
    base_stats = Stats {
        CON = 10,
        VOL = 10,
        INT = 10
    },
    effects = [
        wicca,
        wicca2
    ]
    final_stats = wicca.transform(base_stats)
}