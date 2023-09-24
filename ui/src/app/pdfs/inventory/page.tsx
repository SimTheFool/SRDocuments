import { Effects } from "@/app/_Effects";
import { Identities } from "@/app/_Indentities";
import { Monitors } from "@/app/_Monitors";
import { Resources } from "@/app/_Resources";
import { Skills } from "@/app/_Skills";
import { Stats } from "@/app/_Stats";
import { Header } from "@/components/Header";
import { Section } from "@/components/Section";
import { TitleSection } from "@/components/TitleSection";
import { Box, Container, Flex, Grid, Text } from "@radix-ui/themes";
import { characters } from "resources";
import { A4Format } from "../A4Format";
import { Card } from "@/components/Card";
import { TitleMin } from "@/components/TitleMin";
import Masonry from "masonry-layout";
import { use, useEffect, useRef } from "react";
import { MasonryGrid } from "@/components/MasonryGrid";
import { ParagraphStandard } from "@/components/ParagraphStandard";
import { Monitor } from "@/components/Monitor";
import { StatTable } from "@/components/StatTable";
import { Space } from "@/components/Space";
import { Slot } from "@/components/Slot";

const shrimp = characters.shrimp;

export default function Home() {
  return (
    <A4Format border>
      <TitleSection>Inventory</TitleSection>
      <MasonryGrid columns={3}>
        <Box>
          <Flex align={"stretch"}>
            <Card title="drône">
              <TitleMin title={"Packmule"} subtitle={"16000¥"} />
              <ParagraphStandard>
                <Space />
                Grand arthropode mécanique
                <Space />
                <StatTable
                  compact
                  items={[
                    ["Mani", "Acc.", "Interv.", "Vit.Max"],
                    ["3/4", 6, 5, 30],
                  ]}
                />
                <StatTable
                  compact
                  items={[
                    ["Auto.", "Res.", "Sens.", "Blin."],
                    [2, 8, 1, 10],
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
              <Monitor columns={3} hit={12} alwaysCurable />
            </Box>
          </Flex>
          <Slot size="S">Châsse</Slot>
        </Box>
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
