schema Identity:
    alias: str
    quality: str
    price: int
    description: str
    lifestyle: str
    nuyens?: int
    contacts: [Contact]

schema Contact:
    name: str
    description: str
    loyalty: int
    connection: int