import { Sprite as SpriteType } from "resources";
import { InlineMajorAction, InlineMinorAction } from "../Icons/Actions";
import { StatTable } from "../StatTable";
import { CompanionBox } from "./CompanionBox";

type SpriteProps = {
  name: string;
  sprite: SpriteType;
};

export const Sprite = ({ name, sprite }: SpriteProps) => {
  const stats = sprite.stats;

  return (
    <CompanionBox companion={sprite} name={name} type={"sprite"}>
      {stats && (
        <>
          <StatTable
            compact
            items={[
              ["Firew.", "Trait.", "Corr.", "Att."],
              [
                `P+${stats.firewall}`,
                `P+${stats.traitement}`,
                `P+${stats.corruption}`,
                `P+${stats.attaque}`,
              ],
            ]}
          />
          <StatTable
            compact
            items={[
              [
                "Vie",
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
