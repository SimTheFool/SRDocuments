import { Header } from "@/components/Header";
import { Box, Container, Grid } from "@radix-ui/themes";
import { Effects } from "./_Effects";
import { Identities } from "./_Indentities";
import { Monitors } from "./_Monitors";
import { Resources } from "./_Resources";
import { Skills } from "./_Skills";
import { Stats } from "./_Stats";
import { Section } from "@/components/Section";

export default function Home() {
  return (
    <>
      <Header />
      <Grid
        columns="2"
        gap="4"
        style={{
          gridTemplateColumns: "5fr 5fr",
          gridTemplateRows: "1fr",
        }}
        pt={"4"}
        px={"2"}
      >
        <Container>
          <Stats />
          <Resources edgeNb={2} minActionNb={3} />
          <Monitors />
        </Container>

        <Container>
          <Box
            pl={"3"}
            style={{
              borderLeft: "2px solid var(--gray-10)",
            }}
          >
            <Identities />
          </Box>
          <Box
            pl={"3"}
            style={{
              borderLeft: "2px solid var(--gray-10)",
            }}
          >
            <Effects />
          </Box>
        </Container>
      </Grid>
      <Box
        pt={"3"}
        mx={"3"}
        style={{
          borderTop: "2px solid var(--gray-10)",
        }}
      >
        <Skills
          skills={[
            {
              name: "Electronique",
              score: 4,
              master: [],
            },
            {
              name: "Combat rapproché",
              score: 1,
              master: [],
            },
            {
              name: "Furtivité",
              score: 1,
              master: [],
            },
            {
              name: "Athlétisme",
              score: 3,
              master: [],
            },
            {
              name: "Ingénierie",
              score: 6,
              master: [{ name: "Artillerie", bonus: 2 }],
            },
            {
              name: "Pilotage",
              score: 3,
              master: [{ name: "Appareils au sol", bonus: 2 }],
            },
            {
              name: "Technomancie",
              score: 6,
              master: [
                { name: "Compilation", bonus: 2 },
                { name: "Inscription", bonus: 3 },
              ],
            },
          ]}
        />
      </Box>
      <Box
        pt={"3"}
        mx={"3"}
        style={{
          borderTop: "2px solid var(--gray-10)",
        }}
      >
        <Section title="notes"></Section>
      </Box>
    </>
  );
}
