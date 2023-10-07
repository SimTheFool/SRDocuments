import { OtherCompanion as OtherCompanionType } from "resources";
import { StatTable } from "../StatTable";
import { CompanionBox } from "./CompanionBox";
import { InlineMajorAction, InlineMinorAction } from "../Icons/Actions";
import { Monitor } from "../Monitor";

type OtherCompanionProps = {
  name: string;
  otherCompanion: OtherCompanionType;
};

export const OtherCompanion = ({
  name,
  otherCompanion,
}: OtherCompanionProps) => {
  const stats = otherCompanion.stats;

  return (
    <CompanionBox companion={otherCompanion} name={name} type={"esprit"}>
      {stats && (
        <>
          <StatTable
            compact
            items={[
              ["CON", "AGI", "REA", "FOR"],
              [stats.con, stats.agi, stats.rea, stats.for],
            ]}
          />
          <StatTable
            compact
            items={[
              ["VOL", "LOG", "INT", "CHA"],
              [stats.vol, stats.log, stats.int, stats.cha],
            ]}
          />
          <StatTable
            compact
            items={[
              [
                "Puiss.",
                <InlineMajorAction size={10} />,
                <InlineMinorAction size={12} />,
              ],
              [`${stats.pui}`, `${stats.action_maj}`, `${stats.action_min}`],
            ]}
          />
          <Monitor columns={stats.hit} hit={stats.hit} alwaysCurable />
        </>
      )}
    </CompanionBox>
  );
};
