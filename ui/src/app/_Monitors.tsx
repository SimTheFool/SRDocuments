import { MasonryGrid } from "@/components/MasonryGrid";
import { Monitor } from "@/components/Monitor";
import { Grid, Container, Box } from "@radix-ui/themes";
import { Character } from "resources";

type MonitorsProps = {
  char: Character;
};

export const Monitors = ({ char }: MonitorsProps) => {
  return (
    <>
      <Box pt={"2"}>
        <MasonryGrid columns={2}>
          <Monitor
            columns={4}
            hit={char.stats.hit_phy}
            title={"Dom. Physique"}
          />
          <Monitor
            columns={4}
            hit={char.stats.hit_stun}
            title={"Dom. Etourdissant"}
          />
          <Monitor columns={6} hit={char.stats.hit_over} title={"Surplus"} />
        </MasonryGrid>
      </Box>
    </>
  );
};
