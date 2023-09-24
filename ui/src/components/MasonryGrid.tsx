"use client";

import { Box } from "@radix-ui/themes";
import Masonry from "masonry-layout";
import React from "react";
import { useRef, useEffect } from "react";

const GRID_ITEM_CLASS = "masonryGridItem";
const GRID_ITEM_SIZER_CLASS = "masonryGridSizer";

type MasonryGridProps = {
  columns: number;
  children?: React.ReactNode;
};

export const MasonryGrid = ({ children, columns }: MasonryGridProps) => {
  const gridRef = useRef<HTMLDivElement>(null);
  const columnWidth = 100 / columns;

  useEffect(() => {
    if (!gridRef.current) return;
    new Masonry(gridRef.current, {
      itemSelector: `.${GRID_ITEM_CLASS}`,
      columnWidth: `.${GRID_ITEM_SIZER_CLASS}`,
      percentPosition: true,
    });
  }, [gridRef.current]);

  return (
    <Box ref={gridRef}>
      <Container width={columnWidth} sizer />

      {React.Children.map(children, (child, i) => {
        return (
          <Container key={i} width={columnWidth}>
            {child}
          </Container>
        );
      })}
    </Box>
  );
};

const Container = ({
  children,
  width,
  sizer = false,
}: {
  children?: React.ReactNode;
  width: number;
  sizer?: boolean;
}) => {
  return (
    <Box
      className={sizer ? GRID_ITEM_SIZER_CLASS : GRID_ITEM_CLASS}
      style={{
        visibility: sizer ? "hidden" : "visible",
        width: `${width}%`,
      }}
      px={"1"}
      pb={"1"}
    >
      {children}
    </Box>
  );
};
