import { Box, Flex } from "@radix-ui/themes";
import React from "react";

type FlexListProps = {
  children?: React.ReactNode;
};

export const FlexList = ({ children }: FlexListProps) => {
  return (
    <Flex wrap={"wrap"}>
      {React.Children.map(children, (child) => (
        <Box pl={"2"}>{child}</Box>
      ))}
    </Flex>
  );
};
