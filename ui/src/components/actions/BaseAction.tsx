import { capitalize } from "@/utils/capitalize";
import { Box, Flex } from "@radix-ui/themes";
import { PiDiamondLight } from "react-icons/pi";
import { BaseAction1 } from "resources";
import { Card } from "../Card";
import { MajorAction, MinorAction } from "../Icons/Actions";
import { ParagraphStandard } from "../ParagraphStandard";
import { Ruler } from "../Ruler";
import { Space } from "../Space";
import { TitleMin } from "../TitleMin";
import { BsHourglass } from "react-icons/bs";
import { TextReplaced } from "../Text";

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
                {maintained && <BsHourglass />}
              </>
            }
            inline
          />
          <Space />
          {gauge &&
            Array.from({ length: gauge }).map((_, i) => (
              <PiDiamondLight key={i} />
            ))}
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
