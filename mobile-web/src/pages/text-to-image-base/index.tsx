import {
  Button,
  Divider,
  Dropdown,
  Mask,
  ProgressCircle,
  Radio,
  Selector,
  Slider,
  Space,
  Stepper,
  Image,
  Switch,
  TextArea,
  Grid,
  ImageViewer,
} from 'antd-mobile';
import styles from './index.module.less';
import { useEffect, useState } from 'react';
import { comfyuiAddPrompt, comfyuiHistory, comfyuiQueue, comfyuiView } from '@/api';
import { AddPromptReq, QueueRsp } from '@/typings/api';

// 绘图风格
const paintingStyleOptions = [
  { label: '写实', value: 0 },
  { label: '二次元', value: 1 },
  { label: '国风', value: 2 },
  { label: '卡通', value: 3 },
  { label: '3D角色', value: 4 },
  { label: '插画', value: 5 },
];
// 图片数量
const paintingCountOptions = [
  { label: '1', value: 1 },
  { label: '2', value: 2 },
  { label: '3', value: 3 },
  { label: '4', value: 4 },
];
// 图像尺寸
const imageSizeOptions = [
  { label: '512x512 (1:1)', value: '512x512' },
  { label: '768x512 (1.5:1)', value: '768x512' },
  { label: '960x512 (1.875:1)', value: '960x512' },
  { label: '1024x512 (2:1)', value: '1024x512' },
  { label: '1024x567 (1.778:1)', value: '1024x567' },
  { label: '1536x640 (2.4:1)', value: '1536x640' },
  { label: '1344x768 (1.75:1)', value: '1344x768' },
  { label: '1216x832 (1.46:1)', value: '1216x832' },
  { label: '1152x896 (1.286:1)', value: '1152x896' },
  { label: '1024x1024 (1:1)', value: '1024x1024' },
];

enum QueueStatus {
  Pending,
  Running,
  Complete,
}

export default function AiPaintingTextToImageBase() {
  const [state, setState] = useState<AddPromptReq>({
    style: 0,
    batch_size: 1,
    width: 512,
    height: 512,
    positive: '1girl',
    negative: 'nsfw, nude, porn, without cloth, weapon, blood, bloody, violence',
  });

  const [negativeState, setNegativeState] = useState(false);
  const [imageSizeReversalState, setImageSizeReversalState] = useState(false);
  const [currentPromptIdState, setCurrentPromptIdState] = useState('');
  const [currentQueueState, setCurrentQueueState] = useState<QueueRsp>({
    queue_pending_index: 0,
    queue_pending_total: 0,
    status: QueueStatus.Complete,
  });
  const [ouputImagesState, setOuputImagesState] = useState<string[]>([]);
  const [imageViewState, setImageViewState] = useState({
    visible: false,
    image: '',
  });

  useEffect(() => {
    const handleQueue = async () => {
      const resp = await comfyuiQueue({ prompt_id: currentPromptIdState });

      if (
        resp.queue_pending_index === 0 &&
        resp.status === QueueStatus.Complete &&
        currentQueueState.status === QueueStatus.Complete
      ) {
        // 获取图片
        const resp = await comfyuiHistory({ prompt_id: currentPromptIdState });
        const imgs = [];
        for (const item of resp) {
          const imgResp = await comfyuiView({
            filename: item.filename,
            type: item.type,
            subfolder: item.subfolder,
          });
          console.log(imgResp);
          const uint8Array = new Uint8Array(imgResp);
          const blob = new Blob([uint8Array], { type: 'image/png' });
          const url = URL.createObjectURL(blob);
          imgs.push(url);
        }
        setOuputImagesState([...imgs, ...ouputImagesState]);
        return;
      }

      setCurrentQueueState({
        status: resp.status as QueueStatus,
        queue_pending_index: resp.queue_pending_index,
        queue_pending_total: resp.queue_pending_total,
      });
    };

    handleQueue();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [currentPromptIdState, currentQueueState]);

  const handleStyle = (style: number[]) => {
    setState({
      ...state,
      style: style[0],
    });
  };

  const handleCount = (batchSize: number[]) => {
    setState({
      ...state,
      batch_size: batchSize[0],
    });
  };

  const handleImageSize = (sizeText: string | number) => {
    if (typeof sizeText !== 'string') {
      return;
    }
    const widthAndHeight = sizeText.split('x');
    const width = Number(widthAndHeight[0]);
    const height = Number(widthAndHeight[1]);
    setState({
      ...state,
      width,
      height,
    });
  };

  const handleImageSizeReversal = (v: boolean) => {
    setImageSizeReversalState(v);
    setState({
      ...state,
      width: state.height as number,
      height: state.width as number,
    });
  };

  const handleImageSizeByWidth = (width: number | [number, number]) => {
    if (typeof width !== 'number') {
      return;
    }
    setState({
      ...state,
      width: width,
    });
  };

  const handleImageSizeByHeight = (height: number | [number, number]) => {
    if (typeof height !== 'number') {
      return;
    }
    setState({
      ...state,
      height: height,
    });
  };

  const handleNegativeState = (v: boolean) => {
    setNegativeState(v);
  };

  const handlePropmt = (positive: string) => {
    setState({
      ...state,
      positive,
    });
  };

  const handleNegative = (negative: string) => {
    setState({
      ...state,
      negative,
    });
  };

  const handleSubmit = async () => {
    const resp = await comfyuiAddPrompt(state);
    console.log(resp.prompt_id);
    setCurrentPromptIdState(resp.prompt_id);
  };

  const processText = () => {
    if (currentQueueState.status === QueueStatus.Pending) {
      return `${currentQueueState.queue_pending_index} / ${currentQueueState.queue_pending_total}`;
    }
    if (currentQueueState.status === QueueStatus.Running) {
      return 'Running...';
    }
    return '';
  };

  return (
    <>
      <div>
        {/* 配置选项 */}
        <div>
          <div>
            <h2 className='text-xl mb-2'>绘图风格:</h2>
            <Selector
              style={{
                '--border-radius': '50px',
                '--border': 'solid transparent 1px',
                '--checked-border': 'solid var(--adm-color-primary) 1px',
                '--padding': '8px 18px',
              }}
              showCheckMark={false}
              options={paintingStyleOptions}
              defaultValue={[0]}
              onChange={handleStyle}
            />
          </div>

          <Divider />

          <div>
            <h2 className='text-xl mb-2'>图片数量:</h2>
            <Selector
              style={{
                '--border': 'solid transparent 1px',
                '--checked-border': 'solid var(--adm-color-primary) 1px',
                '--padding': '8px 18px',
              }}
              showCheckMark={false}
              options={paintingCountOptions}
              defaultValue={[1]}
              onChange={handleCount}
            />
          </div>

          <Divider />

          <div>
            <h2 className='text-xl mb-2'>图像尺寸:</h2>
            <Dropdown>
              <Dropdown.Item className='!flex-none' key='sorter' title='预制尺寸'>
                <div style={{ padding: 12 }}>
                  <Radio.Group defaultValue='512x512' onChange={handleImageSize}>
                    <Space direction='vertical' block>
                      {imageSizeOptions.map((item) => (
                        <Radio key={item.label} block value={item.value}>
                          {item.value}
                        </Radio>
                      ))}
                    </Space>
                  </Radio.Group>

                  <div className='mt-2'>
                    <label className='text-xl'>反转</label>
                    <Switch className='ml-2' checked={imageSizeReversalState} onChange={handleImageSizeReversal} />
                  </div>
                </div>
              </Dropdown.Item>
            </Dropdown>

            <div style={{ paddingLeft: '12px' }}>
              <div className='flex items-center'>
                <label className='flex-none w-6'>宽:</label>
                <Slider
                  className='flex-auto'
                  step={64}
                  min={512}
                  max={2048}
                  defaultValue={512}
                  value={state.width}
                  onChange={handleImageSizeByWidth}
                />
                <Stepper
                  min={512}
                  max={2048}
                  defaultValue={512}
                  value={state.width}
                  onChange={handleImageSizeByWidth}
                />
                <label className='flex-none w-8 ml-2'>px</label>
              </div>
              <div className='flex items-center mt-2'>
                <label className='flex-none w-6'>高:</label>
                <Slider
                  className='flex-auto'
                  step={64}
                  min={512}
                  max={2048}
                  defaultValue={512}
                  value={state.height}
                  onChange={handleImageSizeByHeight}
                />
                <Stepper
                  min={512}
                  max={2048}
                  defaultValue={512}
                  value={state.height}
                  onChange={handleImageSizeByHeight}
                />
                <label className='flex-none w-8 ml-2'>px</label>
              </div>
            </div>
          </div>

          <Divider />

          <div>
            <h2 className='text-xl'>提示词:</h2>
            <label className=''>正面提示词</label>
            <TextArea
              showCount
              maxLength={1000}
              value={state.positive}
              style={{ border: '1px solid #e5e7eb', padding: '5px' }}
              onChange={handlePropmt}
            />

            <div className='mt-2 flex items-center'>
              <label className=''>负面提示词</label>
              <Switch className='ml-2' checked={negativeState} onChange={handleNegativeState} />
            </div>
            {negativeState ? (
              <TextArea
                showCount
                maxLength={1000}
                value={state.negative}
                style={{ border: '1px solid #e5e7eb', padding: '5px' }}
                onChange={handleNegative}
              />
            ) : (
              <></>
            )}
          </div>

          <Divider />

          <div>
            <Button block color='primary' size='large' onClick={handleSubmit}>
              开始生图
            </Button>

            <Mask visible={currentQueueState.status !== QueueStatus.Complete}>
              <div className={styles.overlayContent}>
                <ProgressCircle percent={currentQueueState.queue_pending_index / currentQueueState.queue_pending_total}>
                  {processText()}
                </ProgressCircle>
              </div>
            </Mask>
          </div>

          <Divider />

          <div>
            <label>输出图像</label>
            <Grid columns={4} gap={8}>
              {ouputImagesState.map((v, index) => {
                return (
                  <Grid.Item
                    key={index}
                    onClick={() => {
                      setImageViewState({
                        visible: true,
                        image: v,
                      });
                    }}
                  >
                    <Image src={v} width={100} height={100} fit='contain' />
                  </Grid.Item>
                );
              })}
            </Grid>

            <ImageViewer
              classNames={{
                mask: 'customize-mask',
                body: 'customize-body',
              }}
              image={imageViewState.image}
              visible={imageViewState.visible}
              onClose={() => {
                setImageViewState({ visible: false, image: '' });
              }}
            />
          </div>
        </div>
      </div>
    </>
  );
}
