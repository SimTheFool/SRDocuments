import { Monitor } from "@/components/Monitor";
import { Grid, Container } from "@radix-ui/themes";
import { Character } from "resources";

type MonitorsProps = {
  char: Character;
};

export const Monitors = ({ char }: MonitorsProps) => {
  return (
    <>
      <Grid pt={"2"} columns="2" gap="2" align={"start"}>
        <Monitor
          hit={char.stats.hit_phy}
          overHit={char.stats.hit_over}
          title={"Dom. Physique"}
        />
        <Monitor hit={char.stats.hit_stun} title={"Dom. Etourdissant"} />
      </Grid>

      <Container pt={"2"}></Container>
    </>
  );
};
