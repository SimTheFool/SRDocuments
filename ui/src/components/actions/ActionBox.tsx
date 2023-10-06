import { capitalize } from "@/utils/capitalize";
import { Box, Flex } from "@radix-ui/themes";
import { ReactElement, ReactNode } from "react";
import { Card } from "../Card";
import { Space } from "../Space";
import { TextReplaced } from "../Text";
import { TitleMin } from "../TitleMin";
import React from "react";

type ActionBoxProps = {
  title?: string;
  subtitle?: ReactNode;
  type?: ReactNode;
  children: {
    resources?: ReactElement;
    content?: ReactNode;
  };
};
export const ActionBox = ({
  children: { content, resources },
  subtitle,
  title,
  type,
}: ActionBoxProps) => {
  const resourcesChildren = React.Children.toArray(
    resources?.props.children
  ).filter((x) => x);

  return (
    <Card title={type}>
      <Flex justify={"between"}>
        <Box>
          <TitleMin
            title={<TextReplaced>{capitalize(title || "")}</TextReplaced>}
            subtitle={subtitle}
            inline
          />
          <Space />
          {content}
        </Box>
        {resourcesChildren?.length > 0 && (
          <Box>
            {resourcesChildren.map((r) => (
              <Box pb={"1"}>{r}</Box>
            ))}
          </Box>
        )}
      </Flex>
    </Card>
  );
};
