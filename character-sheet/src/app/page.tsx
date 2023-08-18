import { Card } from "@/components/Card";
import { FlexList } from "@/components/FlexList";
import { Header } from "@/components/Header";

import { InlineMonitor, Monitor } from "@/components/Monitor";
import { Section } from "@/components/Section";
import { StatTable } from "@/components/StatTable";
import { TextStandard } from "@/components/TextStandard";
import { TitleMin } from "@/components/TitleMin";
import { Box, Container, Grid, Heading, Text } from "@radix-ui/themes";

export default function Home() {
  return (
    <>
      <Header />
      <Grid
        columns="2"
        gap="2"
        style={{
          gridTemplateColumns: "4fr 3fr",
          gridTemplateRows: "1fr",
        }}
        pt={"4"}
        px={"2"}
      >
        <Section title="Stats">
          <FlexList>
            <StatTable
              items={[
                ["CON", "AGI", "REA", "FOR"],
                ["1", "4", "4", "4"],
              ]}
            />
            <StatTable
              items={[
                ["RES", "ESS", "Subm."],
                ["1", "4", "1"],
              ]}
            />
            <StatTable
              items={[
                ["CON", "AGI", "REA", "FOR"],
                ["1", "4", "4", "4"],
              ]}
            />
            <StatTable items={[["Res"], ["CON(1)"]]} />
            <StatTable items={[["Res"], ["VOL(2)"]]} />
          </FlexList>

          <Grid pt={"4"} pl={"2"} columns="2" gap="2" align={"start"}>
            <Monitor hit={10} title={"Dom. Physique"} />
            <Monitor hit={8} title={"Dom. Etourdissant"} />
          </Grid>

          <Container pt={"4"} pl={"2"}>
            <InlineMonitor hit={5} title={"Surplus"} />
          </Container>
        </Section>

        <Container>
          <Section title="Identités">
            <FlexList paddingBottom grow>
              <Card title={"Nuyens"}>
                <TextStandard>_______</TextStandard>
              </Card>
              <Card title={"Style de vie"}>
                <TextStandard>Squatteur</TextStandard>
              </Card>
              <Card title={"Détails"}>
                <TitleMin
                  title={"Laurence Guinvite"}
                  subtitle={"SIN I4 - 5000Y"}
                />
                <TextStandard>1m80, 80kg, 18 ans. Coursier</TextStandard>
              </Card>

              <Card title={"Contact"}>
                <TitleMin title={"D-Boss"} subtitle={"L4 - R4"} />
                <TextStandard>Decker fan de complot</TextStandard>
              </Card>
              <Card title={"Contact"}>
                <TitleMin title={"Terrance"} subtitle={"L3 - R2"} />
                <TextStandard>Ouvrier de casse militaire d'ARES</TextStandard>
              </Card>
            </FlexList>
          </Section>
        </Container>
      </Grid>
    </>
  );
}
