import { MasonryGrid } from "@/components/MasonryGrid";
import { Space } from "@/components/Space";
import { TitleSection } from "@/components/TitleSection";
import { Drone } from "@/components/items/Drone";
import { Box } from "@radix-ui/themes";
import { Character, characters } from "resources";
import { A4Format } from "../A4Format";
import { ItemCard } from "@/components/items/ItemCard";

const shrimp = characters.shrimp;

const drones = shrimp.drones;
const weapons = shrimp.weapons;

type Weapon = Exclude<Character["weapons"], undefined>[string];

export default function Home() {
  return (
    <A4Format border>
      <Box pt={"2"}>
        <MasonryGrid columns={3}>
          <Box>
            <TitleSection>Inventory</TitleSection>
            <Space />
          </Box>
          {Object.entries(drones || {}).map(([name, drone]) => {
            return (
              <Box pb={"4"} pr={"2"}>
                <Drone item={drone} name={name} />
              </Box>
            );
          })}
          {Object.entries(weapons || {}).map(([name, weapon]) => {
            return (
              <Box pb={"4"} pr={"2"}>
                {" "}
                <ItemCard item={weapon} name={name} />
              </Box>
            );
          })}
        </MasonryGrid>
      </Box>
      {/* <Grid
        columns="2"
        gap="2"
        style={{
          gridTemplateColumns: "58% 42%",
          gridTemplateRows: "1fr",
        }}
        pt={"1"}
        px={"2"}
      >
        <Box>
          <Stats char={shrimp} />
          <Resources char={shrimp} />
          <Monitors char={shrimp} />
        </Box>

        <Box>
          <Box
            pl={"2"}
            style={{
              borderLeft: "2px solid var(--gray-10)",
            }}
          >
            <Identities char={shrimp} />
          </Box>
          <Box
            pl={"2"}
            style={{
              borderLeft: "2px solid var(--gray-10)",
            }}
          >
            <Skills char={shrimp} />
          </Box>
        </Box>
      </Grid> */}
      {/* <Box
        pt={"3"}
        mx={"3"}
        style={{
          borderTop: "2px solid var(--gray-10)",
        }}
      >
        <Effects char={shrimp} />
      </Box>
      <Box
        pt={"3"}
        mx={"3"}
        style={{
          borderTop: "2px solid var(--gray-10)",
        }}
      >
        <Section title={<TitleSection>Notes</TitleSection>}></Section>
      </Box> */}
    </A4Format>
  );
}
