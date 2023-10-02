import { capitalize } from "@/utils/capitalize";
import { Box, Flex } from "@radix-ui/themes";
import { PiDiamondLight, PiHourglassSimpleLowFill } from "react-icons/pi";
import { BaseAction1 } from "resources";
import { Card } from "../Card";
import { Gauge } from "../Gauge";
import { MajorAction, MinorAction } from "../Icons/Actions";
import { BaseIcon } from "../Icons/BaseIcon";
import { ParagraphStandard } from "../ParagraphStandard";
import { Ruler } from "../Ruler";
import { Space } from "../Space";
import { TextReplaced } from "../Text";
import { TitleMin } from "../TitleMin";

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
  return (
    <Card title={type || null}>
      <Flex justify={"between"}>
        <Box>
          <TitleMin
            title={<TextReplaced>{capitalize(name)}</TextReplaced>}
            subtitle={
              <>
                {` ${damage || ""}${damage_type || ""}`}
                {maintained && (
                  <BaseIcon inline>
                    <PiHourglassSimpleLowFill />
                  </BaseIcon>
                )}
              </>
            }
            inline
          />
          <Space />
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
        </Box>
        <Box>
          {Array.from({ length: major }).map((_, i) => (
            <MajorAction key={i} />
          ))}
          {Array.from({ length: minor }).map((_, i) => (
            <MinorAction key={i} />
          ))}
        </Box>
      </Flex>
    </Card>
  );
};
