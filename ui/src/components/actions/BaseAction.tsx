import { Flex, Box } from "@radix-ui/themes";
import { Card } from "../Card";
import { MajorAction, MinorAction } from "../Icons/Actions";
import { ParagraphStandard } from "../ParagraphStandard";
import { Space } from "../Space";
import { TitleMin } from "../TitleMin";
import { BaseAction1 } from "resources";
import { capitalize } from "@/utils/capitalize";
import { PiDiamondLight } from "react-icons/pi";

type BaseActionProps = {
  name: string;
  action: BaseAction1;
};
export const BaseAction = ({
  name,
  action: { major, minor, description, maintained, gauge },
}: BaseActionProps) => {
  return (
    <Card>
      <Flex justify={"between"}>
        <Box>
          <TitleMin title={capitalize(name)} />
          <Space />
          {gauge &&
            Array.from({ length: gauge }).map((_, i) => (
              <PiDiamondLight key={i} />
            ))}
          {description && <ParagraphStandard>{description}</ParagraphStandard>}
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
