import math

schema Base:
    FOR: int
    AGI: int
    REA: int
    CON: int
    CHA: int
    LOG: int
    INT: int
    VOL: int
    Edge: int
    AddInit: int
    InitDices: int = REA + INT + AddInit
    MinActions: int = 1 + AddInit
    NatHeal: int = VOL + CON
    PhysHit: int = 8 + math.ceil(CON / 2)
    StunHit: int = 8 + math.ceil(VOL / 2)
    OverHit: int = CON
 
 
schema Mag[trad: str]:
    class: str
    base: Base
    MAG: int
    Init: int
    ResistDrain: int = math.ceil(base[trad] / 2)

schema Techno:
    class: str
    base: Base
    Res: int
    Subm: int
    ResistDrain: int = math.ceil(base["LOG"] / 2)

schema Regular:
    class: str
    base: Base
