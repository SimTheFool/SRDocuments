import { FlexList } from "@/components/FlexList";
import { Section } from "@/components/Section";
import { StatTable } from "@/components/StatTable";
import { TextIndice } from "@/components/TextIndice";
import { TitleSection } from "@/components/TitleSection";
import { Box, Table } from "@radix-ui/themes";
import { Character } from "resources";

type StatsProps = {
  char: Character;
};

export const Stats = ({ char }: StatsProps) => {
  const stats = char.stats;
  return (
    <Section title={<TitleSection>Stats</TitleSection>}>
      <Box
        pr={"2"}
        display={"inline"}
        style={{
          float: "left",
        }}
      >
        <StatTable
          items={[
            ["CON", "AGI", "REA", "FOR"],
            [
              <StatBlock n={stats.con} mod={stats.con_mod} />,
              <StatBlock n={stats.agi} mod={stats.agi_mod} />,
              <StatBlock n={stats.rea} mod={stats.rea_mod} />,
              <StatBlock n={stats.for} mod={stats.for_mod} />,
            ],
          ]}
        />
        <StatTable
          items={[
            ["VOL", "LOG", "INT", "CHA"],
            [
              <StatBlock n={stats.vol} mod={stats.vol_mod} />,
              <StatBlock n={stats.log} mod={stats.log_mod} />,
              <StatBlock n={stats.int} mod={stats.int_mod} />,
              <StatBlock n={stats.cha} mod={stats.cha_mod} />,
            ],
          ]}
        />
      </Box>
      <Container>
        {stats.res && (
          <StatTable
            inline
            items={[
              ["ESS", "RES", "Subm."],
              [stats.ess, stats.res, stats.submersion],
            ]}
          />
        )}
      </Container>
      <Container>
        {stats.mag && (
          <StatTable
            items={[
              ["ESS", "MAG", "Init."],
              [stats.ess, stats.mag, stats.initiation],
            ]}
          />
        )}
      </Container>
      <Container>
        {stats.res && (
          <StatTable
            items={[
              ["Algo."],
              [
                <StatBlock
                  n={stats.algo?.score || 0}
                  stat={stats.algo?.stat}
                />,
              ],
            ]}
          />
        )}
      </Container>
      <Container>
        {stats.mag && (
          <StatTable
            items={[
              ["Trad."],
              [
                <StatBlock
                  n={stats.trad?.score || 0}
                  stat={stats.trad?.stat}
                />,
              ],
            ]}
          />
        )}
      </Container>
      <Container>
        <StatTable
          items={[
            ["Res.Phy"],
            [
              <StatBlock
                n={stats.resist_phy.score}
                stat={stats.resist_phy.stat}
                dice={6}
              />,
            ],
          ]}
        />
      </Container>
      <Container>
        <StatTable
          items={[
            ["Gué.Nat"],
            [
              <StatBlock
                n={stats.natural_heal.score}
                stat={stats.natural_heal.stat}
                dice={6}
              />,
            ],
          ]}
        />
      </Container>
      {/*<Box pr={"2"}> 
        {stats.res && (
          <StatTable
            items={[
              ["Algo."],
              [
                <StatBlock
                  n={stats.algo?.score || 0}
                  stat={stats.algo?.stat}
                />,
              ],
            ]}
          />
        )}
        {stats.mag && (
          <StatTable
            items={[
              ["Trad."],
              [
                <StatBlock
                  n={stats.trad?.score || 0}
                  stat={stats.trad?.stat}
                />,
              ],
            ]}
          />
        )}
      </Box>
      <Box pr={"2"}>
        <StatTable
          items={[
            ["Res.Phy"],
            [
              <StatBlock
                n={stats.resist_phy.score}
                stat={stats.resist_phy.stat}
                dice={6}
              />,
            ],
          ]}
        />
      </Box>
      <Box pr={"2"}>
        <StatTable
          items={[
            ["Gué.Nat"],
            [
              <StatBlock
                n={stats.natural_heal.score}
                stat={stats.natural_heal.stat}
                dice={6}
              />,
            ],
          ]}
        />
      </Box> */}
    </Section>
  );
};

type StatBlockProps = {
  n: number;
  mod?: number;
  stat?: string[];
  dice?: number;
};

const StatBlock = ({ n, mod, stat, dice }: StatBlockProps) => {
  return (
    <>
      {dice ? `${n}d${dice}` : n}
      {!!mod ? <>({mod})</> : null}{" "}
      <TextIndice>
        {stat && <>{stat.map((s) => s.toUpperCase()).join("-")} </>}
      </TextIndice>
    </>
  );
};

const Container = ({ children }: { children: React.ReactNode }) => {
  return (
    <Box
      pl={"1"}
      style={{
        display: "inline-block",
      }}
    >
      {children}
    </Box>
  );
};
