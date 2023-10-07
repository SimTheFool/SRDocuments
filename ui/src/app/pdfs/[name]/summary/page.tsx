import { Effects } from "@/app/_Effects";
import { Identities } from "@/app/_Indentities";
import { Monitors } from "@/app/_Monitors";
import { Resources } from "@/app/_Resources";
import { Skills } from "@/app/_Skills";
import { Stats } from "@/app/_Stats";
import { Header } from "@/components/Header";
import { Section } from "@/components/Section";
import { TitleSection } from "@/components/TitleSection";
import { Box, Grid } from "@radix-ui/themes";
import { PdfContainer } from "../../PdfContainer";
import { characters } from "resources";

type Props = {
  params: {
    name: string;
  };
};

export default function Home({ params: { name } }: Props) {
  const char = characters[name];

  return (
    <PdfContainer>
      <Header char={char} />
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
          <Stats char={char} />
          <Resources char={char} />
          <Monitors char={char} />
        </Box>

        <Box>
          <Box
            pl={"2"}
            style={{
              borderLeft: "2px solid var(--gray-10)",
            }}
          >
            <Identities char={char} />
          </Box>
          <Box
            pl={"2"}
            style={{
              borderLeft: "2px solid var(--gray-10)",
            }}
          >
            <Skills char={char} />
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
        <Effects char={char} />
      </Box>
      <Box
        pt={"3"}
        mx={"3"}
        style={{
          borderTop: "2px solid var(--gray-10)",
        }}
      >
        <Section title={<TitleSection>Notes</TitleSection>}>
          {Array.from({ length: 10 }).map(() => (
            <Box
              style={{
                width: "100%",
                height: "calc(25px * var(--scaling)",
                borderBottom: "1px solid var(--gray-8)",
              }}
            />
          ))}
        </Section>
      </Box>
    </PdfContainer>
  );
}
