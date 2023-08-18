import { Monitor, InlineMonitor } from "@/components/Monitor";
import { Grid, Container } from "@radix-ui/themes";

type MonitorsProps = {};

export const Monitors = ({}: MonitorsProps) => {
  return (
    <>
      <Grid pt={"2"} columns="2" gap="2" align={"start"}>
        <Monitor hit={10} title={"Dom. Physique"} />
        <Monitor hit={8} title={"Dom. Etourdissant"} />
      </Grid>

      <Container pt={"2"}>
        <InlineMonitor hit={5} title={"Surplus"} />
      </Container>
    </>
  );
};
