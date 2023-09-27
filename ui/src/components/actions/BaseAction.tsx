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
import styles from "./BaseAction.module.css";

type BaseActionProps = {
  name: string;
  action: BaseAction1;
  topBorder?: boolean;
};
export const BaseAction = ({
  name,
  topBorder = false,
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
    <Card>
      <Flex justify={"between"}>
        <Box>
          <TitleMin
            title={capitalize(name)}
            subtitle={` ${damage || ""}${damage_type || ""}`}
            inline
          />
          <Space />
          {gauge &&
            Array.from({ length: gauge }).map((_, i) => (
              <PiDiamondLight key={i} />
            ))}
          {description && <ParagraphStandard>{description}</ParagraphStandard>}
          {score != undefined && (
            <ParagraphStandard>
              <Ruler items={[score]} />
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
