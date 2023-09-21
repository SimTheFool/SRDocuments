import { Header } from "@/components/Header";
import { Box, Container, Grid } from "@radix-ui/themes";
import { Effects } from "./_Effects";
import { Identities } from "./_Indentities";
import { Monitors } from "./_Monitors";
import { Resources } from "./_Resources";
import { Skills } from "./_Skills";
import { Stats } from "./_Stats";
import { Section } from "@/components/Section";
import { characters } from "resources";

const shrimp = characters.shrimp;

export default function Home() {
  return (
    <>
      <Header char={shrimp} />
      <Grid
        columns="2"
        gap="2"
        style={{
          gridTemplateColumns: "5fr 5fr",
          gridTemplateRows: "1fr",
        }}
        pt={"1"}
        px={"2"}
      >
        <Container>
          <Stats char={shrimp} />
          <Resources char={shrimp} />
          <Monitors char={shrimp} />
        </Container>

        <Container>
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
            <Effects char={shrimp} />
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
        <Skills char={shrimp} />
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
