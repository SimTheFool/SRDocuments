import { Box, Flex } from "@radix-ui/themes";
import React from "react";

type FlexListProps = {
  children?: React.ReactNode;
  paddingBottom?: boolean;
  grow?: boolean;
};

export const FlexList = ({
  children,
  paddingBottom = false,
  grow = false,
}: FlexListProps) => {
  return (
    <Flex wrap={"wrap"} align={"stretch"}>
      {React.Children.map(children, (child) => (
        <Box pr={"2"} pb={paddingBottom ? "2" : "0"} grow={grow ? "1" : "0"}>
          {child}
        </Box>
      ))}
    </Flex>
  );
};
