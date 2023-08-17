import { FlexList } from "@/components/FlexList";
import { Header } from "@/components/Header";
import { InlineMonitor, Monitor } from "@/components/Monitor";
import { Section } from "@/components/Section";
import { StatTable } from "@/components/StatTable";
import { Box, Container, Flex, Grid } from "@radix-ui/themes";

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
      >
        <Container>
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

            {/* <Flex pt={"4"} pl={"2"}>
              <Box
                style={{
                  width: "30px",
                  height: "0px",
                  paddingTop: "100%",
                  backgroundColor: "red",
                  border: "1px solid black",
                }}
              />
            </Flex> */}
          </Section>
        </Container>

        <Container>aaaa</Container>
      </Grid>
    </>
  );
}
