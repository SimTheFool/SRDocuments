import { Spirit as SpiritType } from "resources";
import { StatTable } from "../StatTable";
import { CompanionBox } from "./CompanionBox";

type SpiritProps = {
  name: string;
  spirit: SpiritType;
};

export const Spirit = ({ name, spirit }: SpiritProps) => {
  const stats = spirit.stats;

  return (
    <CompanionBox companion={spirit} name={name} type={"esprit"}>
      {stats && (
        <>
          <StatTable
            compact
            items={[
              ["CON", "AGI", "REA", "FOR"],
              [
                `P${getStatString(stats.con)}`,
                `P${getStatString(stats.agi)}`,
                `P${getStatString(stats.rea)}`,
                `P${getStatString(stats.for)}`,
              ],
            ]}
          />
          <StatTable
            compact
            items={[
              ["Vie", "Act.Maj", "Act.Min"],
              [
                `${stats.hit?.base}+P/${stats.hit?.factor}`,
                `${stats.action_maj}`,
                `${stats.action_min}`,
              ],
            ]}
          />
        </>
      )}
    </CompanionBox>
  );
};

const getStatString = (nb: number) => {
  return nb > 0 ? `+${nb}` : nb < 0 ? `-${nb * -1}` : "";
};
