import { FlexList } from "@/components/FlexList";
import { Section } from "@/components/Section";
import { StatTable } from "@/components/StatTable";
import { TextIndice } from "@/components/TextIndice";
import { Box } from "@radix-ui/themes";
import { Character } from "resources";

type StatsProps = {
  char: Character;
};

export const Stats = ({ char }: StatsProps) => {
  const stats = char.stats;
  return (
    <Section title="Stats">
      <FlexList>
        <Box pr={"2"}>
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
        </Box>
        <Box pr={"2"}>
          {stats.res && (
            <StatTable
              items={[
                ["ESS", "RES", "Subm."],
                [stats.ess, stats.res, stats.submersion],
              ]}
            />
          )}
          {stats.mag && (
            <StatTable
              items={[
                ["ESS", "MAG", "Init."],
                [stats.ess, stats.mag, stats.initiation],
              ]}
            />
          )}
        </Box>
        <Box pr={"2"}>
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
        <Box pr={"2"}>
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
        </Box>
      </FlexList>
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
      <TextIndice>
        {stat && <>{stat.map((s) => s.toUpperCase()).join("-")} </>}
      </TextIndice>
      {dice ? `${n}d${dice}` : n}
      {!!mod ? <>({mod})</> : null}
    </>
  );
};
