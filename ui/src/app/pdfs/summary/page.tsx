import { Effects } from "@/app/_Effects";
import { Identities } from "@/app/_Indentities";
import { Monitors } from "@/app/_Monitors";
import { Resources } from "@/app/_Resources";
import { Skills } from "@/app/_Skills";
import { Stats } from "@/app/_Stats";
import { Header } from "@/components/Header";
import { Section } from "@/components/Section";
import { TitleSection } from "@/components/TitleSection";
import { Box, Container, Grid } from "@radix-ui/themes";
import { characters } from "resources";
import { A4Format } from "../A4Format";

const shrimp = characters.shrimp;

export default function Home() {
  return (
    <A4Format>
      <Header char={shrimp} />
      <Grid
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
      </Grid>
      <Box
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
      </Box>
    </A4Format>
  );
}
