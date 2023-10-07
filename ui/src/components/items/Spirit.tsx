import { Spirit as SpiritType } from "resources";
import { StatTable } from "../StatTable";
import { CompanionBox } from "./CompanionBox";
import { InlineMajorAction, InlineMinorAction } from "../Icons/Actions";

type SpiritProps = {
  name: string;
  spirit: SpiritType;
  ergo?: boolean;
};

export const Spirit = ({ name, spirit, ergo = false }: SpiritProps) => {
  const stats = spirit.stats;

  return (
    <CompanionBox companion={spirit} name={name} type={"esprit"} ergo={ergo}>
      {stats && (
        <>
          <StatTable
            compact
            items={[
              ["con", "agi", "rea", "for"],
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
              [
                "vie",
                <InlineMajorAction size={10} />,
                <InlineMinorAction size={12} />,
              ],
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
