import { interleave } from "@/utils/interleave";
import { PiDiamondLight, PiHourglassSimpleLowFill } from "react-icons/pi";
import { BaseAction1 } from "resources";
import { Gauge } from "../Gauge";
import { MajorAction, MinorAction } from "../Icons/Actions";
import { BaseIcon } from "../Icons/BaseIcon";
import { ParagraphStandard } from "../ParagraphStandard";
import { Ruler } from "../Ruler";
import { Space } from "../Space";
import { TextReplaced } from "../Text";
import { ActionBox } from "./ActionBox";
import { Maintained } from "../Icons/Maintained";

type BaseActionProps = {
  name: string;
  action: BaseAction1;
  type?: string;
};
export const BaseAction = ({
  name,
  type,
  action: {
    major,
    minor,
    description,
    maintained,
    gauge,
    damage,
    damage_type,
    score,
  },
}: BaseActionProps) => {
  const subtitle = [
    maintained && <Maintained />,
    damage && damage_type && (
      <TextReplaced>{`${damage}${damage_type}`}</TextReplaced>
    ),
  ].filter((x) => x);

  return (
    <ActionBox
      title={name}
      subtitle={interleave(subtitle, <Space inline />)}
      type={type}
    >
      {{
        content: (
          <>
            {gauge && <Gauge length={gauge} icon={<PiDiamondLight />} />}
            {description && (
              <ParagraphStandard>
                <TextReplaced>{description}</TextReplaced>
              </ParagraphStandard>
            )}
            {score != undefined && (
              <ParagraphStandard>
                <Ruler grade={[score]} inter={[score]} />
              </ParagraphStandard>
            )}
          </>
        ),
        resources: (
          <>
            {Array.from({ length: major }).map((_, i) => (
              <MajorAction key={i} />
            ))}
            {Array.from({ length: minor }).map((_, i) => (
              <MinorAction key={i} />
            ))}
          </>
        ),
      }}
    </ActionBox>
  );
};
