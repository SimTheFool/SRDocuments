import { FlexList } from "@/components/FlexList";
import { Section } from "@/components/Section";
import { StatTable } from "@/components/StatTable";
import { Box } from "@radix-ui/themes";

type StatsProps = {};

export const Stats = ({}: StatsProps) => {
  return (
    <Section title="Stats">
      <FlexList>
        <Box pr={"2"}>
          <StatTable
            items={[
              ["CON", "AGI", "REA", "FOR"],
              ["1", "4", "4", "4"],
            ]}
          />
        </Box>
        <Box pr={"2"}>
          <StatTable
            items={[
              ["RES", "ESS", "Subm."],
              ["1", "4", "1"],
            ]}
          />
        </Box>
        <Box pr={"2"}>
          <StatTable
            items={[
              ["CON", "AGI", "REA", "FOR"],
              ["1", "4", "4", "4"],
            ]}
          />
        </Box>
        <Box pr={"2"}>
          <StatTable items={[["Res"], ["CON(1)"]]} />
        </Box>
        <Box pr={"2"}>
          <StatTable items={[["Res"], ["VOL(2)"]]} />
        </Box>
      </FlexList>
    </Section>
  );
};
