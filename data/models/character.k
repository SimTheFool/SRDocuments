import .stats
import .identity
import .effect

schema Character:
    name: str
    stats: stats.Mag | stats.Techno | stats.Regular
    identities: [identity.Identity]