import { Card } from "@/components/Card";
import { MasonryGrid } from "@/components/MasonryGrid";
import { Monitor } from "@/components/Monitor";
import { ParagraphStandard } from "@/components/ParagraphStandard";
import { Price } from "@/components/Price";
import { Slot } from "@/components/Slot";
import { Space } from "@/components/Space";
import { StatTable } from "@/components/StatTable";
import { TitleMin } from "@/components/TitleMin";
import { TitleSection } from "@/components/TitleSection";
import { capitalize } from "@/utils/capitalize";
import { Box, Flex } from "@radix-ui/themes";
import { Character, characters } from "resources";
import { A4Format } from "../A4Format";

const shrimp = characters.shrimp;
const drones = shrimp.drones;

type Drone = Exclude<Character["drones"], undefined>[string];
const Drone = ({ drone, name }: { drone: Drone; name: string }) => {
  const stats = drone.stats;
  return (
    <Box>
      <Flex align={"stretch"}>
        <Card title={drone.type} note={`d${drone.concealment}`}>
          <TitleMin
            inline
            title={capitalize(name)}
            subtitle={drone.manufacturer}
          />
          <TitleMin
            subtitle={
              <>
                <Price price={drone.price} />
                <Space inline />
                {!drone.legal ? "Illgégal" : drone.licenced ? "Licencié" : ""}
              </>
            }
          />
          <ParagraphStandard>
            <Space />
            {drone.description}
            <Space />
            <StatTable
              compact
              items={[
                ["Mani", "Acc.", "Interv.", "Vit.Max"],
                [
                  `${stats.maniability_flat}/${stats.maniability_rough}`,
                  stats.acceleration,
                  stats.step,
                  stats.max_speed,
                ],
              ]}
            />
            <StatTable
              compact
              items={[
                ["Auto.", "Res.", "Sens.", "Blin."],
                [stats.autopilot, stats.resistance, stats.sensors, stats.armor],
              ]}
            />
          </ParagraphStandard>
        </Card>
        <Box
          style={{
            maxWidth: "20%",
            width: "20%",
          }}
        >
          <Monitor columns={3} hit={drone.hit} alwaysCurable />
        </Box>
      </Flex>
      <MasonryGrid compact columns={2}>
        {drone.slots?.map((slot) => {
          return (
            <Slot size={slot.size} concealment={slot.concealment}>
              {slot.name}
            </Slot>
          );
        })}
      </MasonryGrid>
    </Box>
  );
};

export default function Home() {
  return (
    <A4Format border>
      <Space />
      <Space />
      <MasonryGrid columns={3}>
        <Box>
          <TitleSection>Inventory</TitleSection>
          <Space />
        </Box>
        {Object.entries(drones || {}).map(([name, drone]) => {
          return <Drone drone={drone} name={name} />;
        })}
      </MasonryGrid>

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
